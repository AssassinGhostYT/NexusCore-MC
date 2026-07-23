use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::{write_varu32, write_vari32};
use crate::item::creative;

pub const ID_CREATIVE_CONTENT: u32 = 145;

pub struct CreativeContent;

impl CreativeContent {
    pub fn new() -> Self {
        CreativeContent
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // 1. Groups count as VarU32 — MUST come before items.
        // The client reads this first; omitting it causes it to read the items
        // count as the groups count and deserync the entire packet.
        // We send 0 groups (no creative categories/tabs).
        write_varu32(&mut buf, 0);

        // 2. Items count as VarU32
        let creative_items = creative::items();
        write_varu32(&mut buf, creative_items.len() as u32);
        
        for (idx, item) in creative_items.iter().enumerate() {
            // CreativeItemNetworkID: VarU32
            write_varu32(&mut buf, (idx as u32) + 1);

            // Item Instance Descriptor:
            // - Network ID (varint32)
            write_vari32(&mut buf, item.network_id);
            // - Count (uint16 LE): 1
            buf.write_u16::<LittleEndian>(1).unwrap();
            // - Metadata (varuint32): 0
            write_varu32(&mut buf, 0);
            // - Block Runtime ID (varint32)
            write_vari32(&mut buf, item.block_runtime_id);

            // Extra data (varuint32 length-prefix + bytes)
            let mut extra = Vec::new();
            // NBT data length (int16 LE): 0 (no NBT)
            extra.write_i16::<LittleEndian>(0).unwrap();
            // CanBePlacedOn count (uint32 LE): 0
            extra.write_u32::<LittleEndian>(0).unwrap();
            // CanBreak count (uint32 LE): 0
            extra.write_u32::<LittleEndian>(0).unwrap();
            
            write_varu32(&mut buf, extra.len() as u32);
            buf.extend_from_slice(&extra);
        }

        buf
    }
}
