use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::{write_varu32, write_vari32};
use super::helpers::write_string;
use crate::item::creative;

pub const ID_CREATIVE_CONTENT: u32 = 145;

pub struct CreativeContent;

impl CreativeContent {
    pub fn new() -> Self {
        CreativeContent
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // 1. Groups slice: 2 groups (anonymous groups for Construction and Nature)
        write_varu32(&mut buf, 2);
        
        // Group 0: Construction (Category 1)
        buf.write_i32::<LittleEndian>(1).unwrap();
        write_string(&mut buf, "");
        write_vari32(&mut buf, 0); // Air Icon
        
        // Group 1: Nature (Category 2)
        buf.write_i32::<LittleEndian>(2).unwrap();
        write_string(&mut buf, "");
        write_vari32(&mut buf, 0); // Air Icon

        // 2. Items slice: length of our creative items from registry
        let creative_items = creative::items();
        write_varu32(&mut buf, creative_items.len() as u32);
        
        for (idx, item) in creative_items.iter().enumerate() {
            // - CreativeItemNetworkID: varuint32 (idx + 1)
            write_varu32(&mut buf, (idx as u32) + 1);

            // - Item: ItemStack
            //   - Network ID (varint32)
            write_vari32(&mut buf, item.network_id);
            //   - Count (uint16 LE): 1
            buf.write_u16::<LittleEndian>(1).unwrap();
            //   - Metadata (varuint32): 0
            write_varu32(&mut buf, 0);
            //   - Block Runtime ID (varint32)
            write_vari32(&mut buf, item.block_runtime_id);

            //   - Extra data (varuint32 length-prefix + bytes)
            let mut extra = Vec::new();
            //     - NBT data length (int16 LE): 0 (no NBT)
            extra.write_i16::<LittleEndian>(0).unwrap();
            //     - CanBePlacedOn count (uint32 LE): 0
            extra.write_u32::<LittleEndian>(0).unwrap();
            //     - CanBreak count (uint32 LE): 0
            extra.write_u32::<LittleEndian>(0).unwrap();
            
            //     Write extra data length and bytes
            write_varu32(&mut buf, extra.len() as u32);
            buf.extend_from_slice(&extra);

            // - GroupIndex: varuint32 (Group index based on item category: Category 1 -> GroupIndex 0, Category 2 -> GroupIndex 1)
            let group_idx = if item.category == 1 { 0 } else { 1 };
            write_varu32(&mut buf, group_idx);
        }

        buf
    }
}
