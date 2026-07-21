// JigsawStructureDataPacket — ID 313 (0x139)
// Sent BEFORE StartGame. Contains behavior pack jigsaw structure rules.
// The client uses these for structure-based world generation.
// An empty compound NBT tag is correct for vanilla/no-behavior-pack servers.
//
// Wire format:
//   jigsaw_structure_data_tag: NBT TAG_Compound (little-endian network NBT)
//     Empty compound: 0x0a 0x00 0x00 0x00
//       0x0a = TAG_Compound
//       0x00 0x00 = name length (u16 LE) = 0 (no name on root)
//       0x00 = TAG_End (closes the compound)

use crate::protocol::error::PResult;

/// JigsawStructureData packet — ID 313 (0x139).
///
/// Must be sent **before** StartGame (per Mojang protocol docs).
/// Default: empty compound NBT (no jigsaw structures from behavior packs).
pub struct JigsawStructureData;

impl JigsawStructureData {
    pub fn new() -> Self {
        Self
    }

    pub fn write(&self) -> PResult<Vec<u8>> {
        let mut buf = Vec::new();
        // Network NBT TAG_Compound (0x0a) + unnamed root string length (varuint32 0 = 0x00) + TAG_End (0x00)
        buf.push(0x0a);
        buf.push(0x00);
        buf.push(0x00);
        log::info!("JigsawStructureData::write: bytes count={}, hex={:02x?}", buf.len(), buf);
        Ok(buf)
    }
}
