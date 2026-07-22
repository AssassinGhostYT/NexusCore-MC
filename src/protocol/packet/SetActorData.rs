use crate::protocol::varint::{write_varu64, write_varu32, write_vari64};
use byteorder::{LittleEndian, WriteBytesExt};

// Packet ID for updating actor metadata
pub const ID_SET_ACTOR_DATA: u32 = 39;

pub struct SetActorData {
    // Unique runtime ID of the entity
    pub entity_runtime_id: u64,
    // Current server tick
    pub tick: u64,
}

impl SetActorData {
    // Serializes the packet data to bytes
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // 1. Entity Runtime ID (VarU64)
        write_varu64(&mut buf, self.entity_runtime_id);

        // 2. Metadata (SynchedActorDataList)
        // Write 4 entries to config height, width, scale, and flags
        write_varu32(&mut buf, 4);

        // Entry 0: EntityDataKeyFlags (0) -> Int64 (7)
        write_varu32(&mut buf, 0); // key (varu32)
        buf.push(7);               // type (i8)
        let flags: i64 = (1i64 << 35) | (1i64 << 48) | (1i64 << 49);
        write_vari64(&mut buf, flags);

        // Entry 1: EntityDataKeyScale (42) -> Float (3)
        write_varu32(&mut buf, 42); // key (varu32)
        buf.push(3);                // type (i8)
        buf.write_f32::<LittleEndian>(1.0).unwrap();

        // Entry 2: EntityDataKeyWidth (57) -> Float (3)
        write_varu32(&mut buf, 57); // key (varu32)
        buf.push(3);                // type (i8)
        buf.write_f32::<LittleEndian>(0.6).unwrap();

        // Entry 3: EntityDataKeyHeight (58) -> Float (3)
        write_varu32(&mut buf, 58); // key (varu32)
        buf.push(3);                // type (i8)
        buf.write_f32::<LittleEndian>(1.8).unwrap();

        // 3. Properties (PropertySyncData)
        // Write 0 for both Int and Float entries to signal no synced properties
        write_varu32(&mut buf, 0); // Int entries count
        write_varu32(&mut buf, 0); // Float entries count

        // 4. Tick (PlayerInputTick)
        write_varu64(&mut buf, self.tick);

        buf
    }
}
