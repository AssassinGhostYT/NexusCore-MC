use byteorder::{LittleEndian, ReadBytesExt};
use crate::protocol::error::PResult;
use crate::protocol::varint::{read_vari32, read_varu32};

pub struct InventoryTransactionAction {
    pub source_type: u32,
    pub source_container: u32,
    pub source_slot: u32,
    pub dest_type: u32,
    pub dest_container: u32,
    pub dest_slot: u32,
    pub count: u32,
    pub item_id: i32,
    pub meta: u16,
}

pub struct InventoryTransaction {
    pub legacy_request_id: i32,
    pub actions: Vec<InventoryTransactionAction>,
}

impl InventoryTransaction {
    pub fn read(buf: &mut &[u8]) -> PResult<Self> {
        let legacy_request_id = read_vari32(buf).ok_or_else(|| {
            crate::protocol::error::PacketError::VarintOverflow { kind: "InventoryTransaction.legacy_request_id" }
        })? as i32;
        let legacy_set_item_slots_count = read_varu32(buf).ok_or_else(|| {
            crate::protocol::error::PacketError::VarintOverflow { kind: "InventoryTransaction.legacy_set_item_slots_count" }
        })? as usize;
        for _ in 0..legacy_set_item_slots_count {
            let _container_id = buf.read_u8().map_err(|e| {
                crate::protocol::error::PacketError::Io { context: "InventoryTransaction.container_id", source: e }
            })?;
            let count = read_varu32(buf).ok_or_else(|| {
                crate::protocol::error::PacketError::VarintOverflow { kind: "InventoryTransaction.slot_count" }
            })? as usize;
            for _ in 0..count {
                let _slot = buf.read_u8().map_err(|e| {
                    crate::protocol::error::PacketError::Io { context: "InventoryTransaction.slot", source: e }
                })?;
            }
        }
        let _transaction_type = read_varu32(buf).ok_or_else(|| {
            crate::protocol::error::PacketError::VarintOverflow { kind: "InventoryTransaction.transaction_type" }
        })?;
        let actions_count = read_varu32(buf).ok_or_else(|| {
            crate::protocol::error::PacketError::VarintOverflow { kind: "InventoryTransaction.actions_count" }
        })? as usize;
        let mut actions = Vec::new();
        for _ in 0..actions_count {
            let source_type = read_varu32(buf).ok_or(crate::protocol::error::PacketError::VarintOverflow { kind: "action.source_type" })?;
            let source_container = read_varu32(buf).ok_or(crate::protocol::error::PacketError::VarintOverflow { kind: "action.source_container" })?;
            let source_slot = read_varu32(buf).ok_or(crate::protocol::error::PacketError::VarintOverflow { kind: "action.source_slot" })?;
            let dest_type = read_varu32(buf).ok_or(crate::protocol::error::PacketError::VarintOverflow { kind: "action.dest_type" })?;
            let dest_container = read_varu32(buf).ok_or(crate::protocol::error::PacketError::VarintOverflow { kind: "action.dest_container" })?;
            let dest_slot = read_varu32(buf).ok_or(crate::protocol::error::PacketError::VarintOverflow { kind: "action.dest_slot" })?;
            let count = read_varu32(buf).ok_or(crate::protocol::error::PacketError::VarintOverflow { kind: "action.count" })?;
            let item_id = read_vari32(buf).ok_or(crate::protocol::error::PacketError::VarintOverflow { kind: "action.item_id" })? as i32;
            let meta = buf.read_u16::<LittleEndian>().map_err(|e| {
                crate::protocol::error::PacketError::Io { context: "action.meta", source: e }
            })?;
            actions.push(InventoryTransactionAction { source_type, source_container, source_slot, dest_type, dest_container, dest_slot, count, item_id, meta });
        }
        Ok(Self { legacy_request_id, actions })
    }
}
