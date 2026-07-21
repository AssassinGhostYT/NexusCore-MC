use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::{write_varu32, write_vari32};
use crate::macros::helpers::write_string;

/// ItemStack represents an item instance present over the network protocol.
/// It contains the item's network ID, count, metadata, and optional extra data such as NBT.
#[derive(Debug, Clone)]
pub struct ItemStack {
    /// NetworkID is the unique network ID of the item type.
    pub network_id: i32,
    /// BlockRuntimeID is the runtime ID of the block if the item is a block.
    pub block_runtime_id: i32,
    /// Count is the amount of items present in the stack.
    pub count: u16,
    /// Metadata is the metadata value or damage of the item.
    pub metadata: u32,
}

impl ItemStack {
    pub fn write(&self, buf: &mut Vec<u8>) {
        if self.network_id == 0 {
            // Air items are written as a single 0 varint in modern Bedrock.
            write_vari32(buf, 0);
            return;
        }

        write_vari32(buf, self.network_id);
        buf.write_u16::<LittleEndian>(self.count).unwrap();
        write_varu32(buf, self.metadata);
        write_vari32(buf, self.block_runtime_id);

        // Extra data block (NBT, CanPlaceOn, CanBreak components)
        let mut extra = Vec::new();
        extra.write_i16::<LittleEndian>(0).unwrap(); // NBT length (0 means no NBT)
        extra.write_u32::<LittleEndian>(0).unwrap(); // CanPlaceOn count
        extra.write_u32::<LittleEndian>(0).unwrap(); // CanBreak count
        
        write_varu32(buf, extra.len() as u32);
        buf.extend_from_slice(&extra);
    }
}

/// CreativeGroup represents a group of items in the creative inventory.
/// Each group has a category, name and an icon that represents the group.
#[derive(Debug, Clone)]
pub struct CreativeGroup {
    /// Category is the category the group falls under (e.g., Construction, Nature).
    pub category: i32,
    /// Name is the locale name of the group, i.e. "itemGroup.name.planks".
    pub name: String,
    /// Icon is the item that represents the group in the creative inventory.
    pub icon: ItemStack, 
}

impl CreativeGroup {
    pub fn write(&self, buf: &mut Vec<u8>) {
        buf.write_i32::<LittleEndian>(self.category).unwrap();
        write_string(buf, &self.name);
        self.icon.write(buf);
    }
}

/// CreativeItem represents a creative item present in the creative inventory.
#[derive(Debug, Clone)]
pub struct CreativeItem {
    /// CreativeItemNetworkID is a unique ID for the creative item. It has to be unique
    /// for each creative item sent to the client. An incrementing ID per creative item does the job.
    pub creative_item_network_id: u32,
    /// Item is the item that should be added to the creative inventory.
    pub item: ItemStack,
    /// GroupIndex is the index of the group that the item should be placed in. It is the index
    /// of the group in the CreativeContent packet previously sent to the client.
    pub group_index: u32,
}

impl CreativeItem {
    pub fn write(&self, buf: &mut Vec<u8>) {
        write_varu32(buf, self.creative_item_network_id);
        self.item.write(buf);
        write_varu32(buf, self.group_index);
    }
}
