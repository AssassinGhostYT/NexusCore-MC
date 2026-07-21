// ItemRegistry — ID 162
// Sent by the server to send the client a list of available items.
// Required for mobile/Xbox Live clients to prevent crashes.

use crate::protocol::error::PResult;
use crate::protocol::varint::{write_varu32, write_vari32};
use crate::macros::helpers;
use byteorder::{LittleEndian, WriteBytesExt};
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
struct ItemJson {
    name: String,
    runtime_id: i16,
}

pub struct ItemRegistry {
    data: Vec<u8>,
}

impl ItemRegistry {
    pub fn load_from_json() -> Self {
        let mut data = Vec::new();
        let file = File::open("items.json");
        if let Ok(f) = file {
            let reader = BufReader::new(f);
            let items: Vec<ItemJson> = serde_json::from_reader(reader).unwrap_or_default();
            log::info!("Loaded {} items from items.json for ItemRegistry", items.len());
            
            // 1. Write item count as varuint32
            write_varu32(&mut data, items.len() as u32);
            
            // 2. Write each item
            for item in items {
                // Name as string
                helpers::write_string(&mut data, &item.name);
                // Runtime ID as i16 LE
                data.write_i16::<LittleEndian>(item.runtime_id).unwrap();
                // ComponentBased as bool (false)
                data.push(0);
                // Version as vari32 (0)
                write_vari32(&mut data, 0);
                // Data as empty NBT compound tag [0x0a, 0x00, 0x00]
                data.push(0x0a);
                data.push(0x00);
                data.push(0x00);
            }
        } else {
            log::error!("Failed to open items.json! Sending empty ItemRegistry.");
            write_varu32(&mut data, 0);
        }
        
        Self { data }
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        Ok(self.data.clone())
    }
}
