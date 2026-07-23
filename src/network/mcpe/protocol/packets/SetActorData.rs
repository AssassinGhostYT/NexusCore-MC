use crate::protocol::varint::{write_varu64, write_varu32, write_vari64};

// Packet ID for updating actor metadata (ID 39 / 0x27)
pub const ID_SET_ACTOR_DATA: u32 = 39;

pub const DATA_TYPE_LONG: u32 = 7;
pub const DATA_INDEX_FLAGS: u32 = 0;

pub struct SetActorData {
    pub entity_runtime_id: u64,
    pub tick: u64,
}

impl SetActorData {
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // 1. Entity Runtime ID (VarU64)
        write_varu64(&mut buf, self.entity_runtime_id);

        // 2. Metadata (SynchedActorDataList) -> 1 entry (FLAGS = DATA_TYPE_LONG)
        write_varu32(&mut buf, 1); // 1 entry

        // Entry 0: Key = 0 (DATA_INDEX_FLAGS), Type = 7 (DATA_TYPE_LONG)
        write_varu32(&mut buf, DATA_INDEX_FLAGS);
        write_varu32(&mut buf, DATA_TYPE_LONG);
        write_vari64(&mut buf, 0); // Flags = 0 (i64 zigzag varint)

        // 3. Properties (PropertySyncData)
        write_varu32(&mut buf, 0); // Int entries count = 0
        write_varu32(&mut buf, 0); // Float entries count = 0

        // 4. Tick (PlayerInputTick)
        write_varu64(&mut buf, self.tick);

        buf
    }
}
