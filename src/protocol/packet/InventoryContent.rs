use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::varint::{write_varu32, write_vari32};

pub const ID_INVENTORY_CONTENT: u32 = 49;

#[derive(Debug, Clone)]
pub struct InventoryContent {
    pub window_id: u32,
    pub slots: Vec<(i32, i32, u16)>, // (network_id, block_runtime_id, count)
}

impl InventoryContent {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // 1. WindowID: varuint32
        write_varu32(&mut buf, self.window_id);

        // 2. Content: Slice with varuint32 length prefix
        write_varu32(&mut buf, self.slots.len() as u32);
        for slot in &self.slots {
            write_item_instance_new(&mut buf, slot.0, slot.1, slot.2);
        }

        // 3. Container: FullContainerName
        //   - ContainerID (uint8): 0
        buf.push(0);
        //   - DynamicContainerID Optional[uint32]: false (not present -> 0 byte)
        buf.push(0);

        // 4. StorageItem: ItemInstanceNew (Empty/Air item)
        write_item_instance_new(&mut buf, 0, 0, 0);

        buf
    }
}

fn write_item_instance_new(buf: &mut Vec<u8>, network_id: i32, block_runtime_id: i32, count: u16) {
    if network_id == 0 {
        // Air
        buf.write_i16::<LittleEndian>(0).unwrap(); // ID
        buf.write_u16::<LittleEndian>(0).unwrap(); // Count
        write_varu32(buf, 0); // MetadataValue
        buf.push(0); // hasNetID: false
        write_varu32(buf, 0); // BlockRuntimeID
        write_varu32(buf, 0); // zero (extra data count)
    } else {
        buf.write_i16::<LittleEndian>(network_id as i16).unwrap(); // ID
        buf.write_u16::<LittleEndian>(count).unwrap(); // Count
        write_varu32(buf, 0); // MetadataValue
        buf.push(0); // hasNetID: false
        write_varu32(buf, block_runtime_id as u32); // BlockRuntimeID

        // Extra data prefix length + bytes (10 bytes total)
        let mut extra = Vec::new();
        extra.write_i16::<LittleEndian>(0).unwrap(); // NBT length
        extra.write_u32::<LittleEndian>(0).unwrap(); // CanPlaceOn count
        extra.write_u32::<LittleEndian>(0).unwrap(); // CanBreak count
        
        write_varu32(buf, extra.len() as u32);
        buf.extend_from_slice(&extra);
    }
}
