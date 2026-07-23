use byteorder::{LittleEndian, WriteBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::{write_varu32, write_vari32};

pub struct InventoryContent {
    pub window_id: u32,
    pub slots: Vec<(u32, u32, u32)>,
}

impl InventoryContent {
    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        
        // 1. Inventory Window ID (VarU32)
        write_varu32(&mut buf, self.window_id);
        
        // 2. Slots count (VarU32)
        write_varu32(&mut buf, self.slots.len() as u32);
        for (net_id, count, meta) in &self.slots {
            if *net_id == 0 && *count == 0 && *meta == 0 {
                write_vari32(&mut buf, 0);
            } else {
                write_vari32(&mut buf, *net_id as i32);
                buf.write_u16::<LittleEndian>(*count as u16).unwrap();
                write_varu32(&mut buf, *meta);
            }
        }

        // 3. FullContainerName (Bedrock v1001 addition)
        // Map window_id to exact ContainerEnumName:
        // Window 0 = InventoryContainer (29)
        // Window 119 = ArmorContainer (6)
        // Window 120 = OffhandContainer (34)
        let container_enum: u8 = match self.window_id {
            0 => 29,   // InventoryContainer
            119 => 6,  // ArmorContainer
            120 => 34, // OffhandContainer
            _ => 29,
        };
        buf.push(container_enum);
        // dynamic_id: Option<i32> bool = false (None)
        buf.push(0);

        // 4. storage_item: NetworkItemStackDescriptorV2 (Bedrock v1001 addition)
        // empty item = 0
        write_vari32(&mut buf, 0);

        Ok(buf)
    }
}
