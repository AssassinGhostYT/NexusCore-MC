use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::{write_varu32, write_vari32};
use crate::item::creative;

/// CreativeContentPacket (ID 145 / 0x91)
///
/// Sent by the server after ResourcePackStack is acknowledged to populate
/// the client's creative inventory. Wire layout (PocketMine protocol):
///   1. VarU32  groups_count
///   2. [groups_count × CreativeGroupEntry]
///   3. VarU32  items_count
///   4. [items_count × CreativeItemEntry]
pub struct CreativeContent;

impl CreativeContent {
    pub fn new() -> Self {
        CreativeContent
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // 1. Groups count — MUST come before items.
        // The client reads groups first; omitting this causes it to read
        // the items count as groups count and desyncs the entire packet.
        // We send 0 groups (no creative category tabs).
        write_varu32(&mut buf, 0);

        // 2. Items count + item entries
        let creative_items = creative::items();
        write_varu32(&mut buf, creative_items.len() as u32);

        for (idx, item) in creative_items.iter().enumerate() {
            // CreativeItemNetworkID: VarU32 (1-based)
            write_varu32(&mut buf, (idx as u32) + 1);

            // Item Instance Descriptor:
            // - Network ID (VarI32)
            write_vari32(&mut buf, item.network_id);
            // - Count (u16 LE): 1
            buf.write_u16::<LittleEndian>(1).unwrap();
            // - Metadata (VarU32): 0
            write_varu32(&mut buf, 0);
            // - Block Runtime ID (VarI32)
            write_vari32(&mut buf, item.block_runtime_id);

            // Extra data (VarU32 length-prefixed blob)
            let mut extra = Vec::new();
            // NBT data length (i16 LE): 0 = no NBT
            extra.write_i16::<LittleEndian>(0).unwrap();
            // CanBePlacedOn count (u32 LE): 0
            extra.write_u32::<LittleEndian>(0).unwrap();
            // CanBreak count (u32 LE): 0
            extra.write_u32::<LittleEndian>(0).unwrap();

            write_varu32(&mut buf, extra.len() as u32);
            buf.extend_from_slice(&extra);
        }

        buf
    }
}
