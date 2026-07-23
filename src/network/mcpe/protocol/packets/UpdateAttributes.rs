use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::{write_varu32, write_varu64};
use super::helpers::write_string;

pub const ID_UPDATE_ATTRIBUTES: u32 = 29;

use crate::protocol::types::Attribute;

#[derive(Clone, Debug)]
pub struct UpdateAttributes {
    pub entity_runtime_id: u64,
    pub attributes: Vec<Attribute>,
    pub tick: u64,
}

impl UpdateAttributes {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // 1. Entity Runtime ID (VarU64)
        write_varu64(&mut buf, self.entity_runtime_id);

        // 2. Attributes slice count (VarU32)
        write_varu32(&mut buf, self.attributes.len() as u32);

        for attr in &self.attributes {
            // Min (Float32 LE)
            buf.write_f32::<LittleEndian>(attr.min).unwrap();
            // Max (Float32 LE)
            buf.write_f32::<LittleEndian>(attr.max).unwrap();
            // Current Value (Float32 LE)
            buf.write_f32::<LittleEndian>(attr.value).unwrap();
            // Default Min (Float32 LE) - Added in v729/v1001
            buf.write_f32::<LittleEndian>(attr.min).unwrap();
            // Default Max (Float32 LE) - Added in v729/v1001
            buf.write_f32::<LittleEndian>(attr.max).unwrap();
            // Default Value (Float32 LE)
            buf.write_f32::<LittleEndian>(attr.default).unwrap();
            // Name (String)
            write_string(&mut buf, &attr.name);
            // Modifiers count (VarU32) = 0
            write_varu32(&mut buf, 0);
        }

        // 3. Tick (VarU64)
        write_varu64(&mut buf, self.tick);

        buf
    }
}
